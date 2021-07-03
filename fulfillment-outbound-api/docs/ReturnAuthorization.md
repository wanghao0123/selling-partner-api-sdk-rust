# ReturnAuthorization

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**return_authorization_id** | **String** | An identifier for the return authorization. This identifier associates return items with the return authorization used to return them. | [default to null]
**fulfillment_center_id** | **String** | An identifier for the Amazon fulfillment center that the return items should be sent to. | [default to null]
**return_to_address** | [***::models::Address**](Address.md) | The address of the Amazon fulfillment center that the return items should be sent to. | [default to null]
**amazon_rma_id** | **String** | The return merchandise authorization (RMA) that Amazon needs to process the return. | [default to null]
**rma_page_url** | **String** | A URL for a web page that contains the return authorization barcode and the mailing label. This does not include pre-paid shipping. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


