# TaxWithheldComponent

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tax_collection_model** | **String** | The tax collection model applied to the item.  Possible values:  * MarketplaceFacilitator - Tax is withheld and remitted to the taxing authority by Amazon on behalf of the seller.  * Standard - Tax is paid to the seller and not remitted to the taxing authority by Amazon. | [optional] [default to null]
**taxes_withheld** | [***::models::ChargeComponentList**](ChargeComponentList.md) | A list of charges that represent the types and amounts of taxes withheld. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


