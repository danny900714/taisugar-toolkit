use url::Url;
use crate::{GetItemNeedsOptions, ItemNeeds, OperationCenter};
use crate::error::Error;

pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_operation_centers(&self) -> Result<Vec<OperationCenter>, Error> {
        Ok(self
            .client
            .get("http://192.168.41.30/TSCRED/BulkPeriodSheet/CennoDropdownList")
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_item_needs(
        &self,
        options: GetItemNeedsOptions<'_>,
    ) -> Result<ItemNeeds, Error> {
        let mut url = Url::parse("http://192.168.41.30/TSCRED/ItemNeedCount/GetItemNeedCount")?;
        url.query_pairs_mut()
            .append_key_only("CLANA")
            .append_pair("CLANA2", options.operation_center_id)
            .append_pair(
                "CLANO",
                &options.start_date.strftime("%Y/%m/%d").to_string(),
            )
            .append_pair("CLANO2", &options.end_date.strftime("%Y/%m/%d").to_string())
            .append_pair("DSP_SEL", &options.display_mode.to_string())
            .append_pair("HOST", options.department_id);

        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<ItemNeeds>()
            .await?)
    }
}