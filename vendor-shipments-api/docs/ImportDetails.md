# ImportDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**method_of_payment** | **String** | This is used for import purchase orders only. If the recipient requests, this field will contain the shipment method of payment. | [optional] [default to null]
**seal_number** | **String** | The container&#39;s seal number. | [optional] [default to null]
**route** | [***::models::Route**](Route.md) | The route and stop details for this shipment. | [optional] [default to null]
**import_containers** | **String** | Types and numbers of container(s) for import purchase orders. Can be a comma-separated list if shipment has multiple containers. | [optional] [default to null]
**billable_weight** | [***::models::Weight**](Weight.md) | Billable weight of the direct imports shipment. | [optional] [default to null]
**estimated_ship_by_date** | **String** | Date on which the shipment is expected to be shipped. This value should not be in the past and not more than 60 days out in the future. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


