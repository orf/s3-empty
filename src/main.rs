use rusoto_s3::{S3Client, S3, ListObjectsRequest, GetBucketLocationRequest};
use rusoto_core::Region;
use structopt::StructOpt;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt()]
    region: String,
    #[structopt()]
    bucket: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    let opt: Opt = Opt::from_args();
    let region = Region::from_str(opt.region.as_str())?;
    let s3_client = S3Client::new(region);
    Ok(())
    // client.list_objects_v2(ListObjectsRequest {
    //     bucket: "".to_string(),
    //     delimiter: None,
    //     encoding_type: None,
    //     marker: None,
    //     max_keys: None,
    //     prefix: None,
    //     request_payer: None
    // })
}
