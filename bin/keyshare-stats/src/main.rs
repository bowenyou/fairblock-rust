use std::{fs::File, io::Write};

use anyhow::{anyhow, Result};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use fairblock_proto::fairyring::keyshare::QueryAllAggregatedKeyShareRequest;
use tonic::transport::Channel;

#[derive(Debug, serde::Serialize)]
struct Keyshares {
    _height: u64,
    _keyshares: String,
}

fn export_to_json_file(all_key_shares: &[Keyshares]) -> Result<()> {
    let file =
        File::create("keyshares.json").map_err(|e| anyhow!("Failed to create file: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, all_key_shares)
        .map_err(|e| anyhow!("Failed to write JSON: {}", e))?;

    writer
        .flush()
        .map_err(|e| anyhow!("Failed to flush file: {}", e))?;

    println!("Data exported to 'keyshares.json'");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let channel = Channel::from_static("http://54.210.57.197:9090")
        .connect()
        .await
        .unwrap();

    let mut all_key_shares = Vec::<Keyshares>::new();

    let mut keyshare_query_client =
        fairblock_proto::fairyring::keyshare::query_client::QueryClient::new(channel.clone());
    let mut pagination = Some(PageRequest {
        key: vec![],
        offset: 0,
        limit: 100,
        count_total: false,
        reverse: false,
    });

    loop {
        let request = QueryAllAggregatedKeyShareRequest {
            pagination: pagination.clone(),
        };

        let response = keyshare_query_client
            .aggregated_key_share_all(request)
            .await?
            .into_inner();

        all_key_shares.extend(response.aggregated_key_share.iter().map(|key| Keyshares {
            _height: key.height,
            _keyshares: key.data.to_string(),
        }));

        if response
            .pagination
            .as_ref()
            .expect("shouldn't fail")
            .next_key
            .len()
            == 0
        {
            break;
        }

        pagination = response.clone().pagination.map(|p| PageRequest {
            key: p.next_key,
            offset: 0,
            limit: 100,
            count_total: false,
            reverse: false,
        });

        println!("{:?}", response.aggregated_key_share.last());

        println!("Collected {:?} values", all_key_shares.len());
    }

    export_to_json_file(&all_key_shares)?;

    Ok(())
}
