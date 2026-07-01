Agrega documentos agrupándolos (si se proporciona **groupBy**) y aplicando múltiples operaciones. Se admiten diferentes operaciones (p. ej., sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| aggregationRequest | AggregationRequest | Sí |  |
| options | const AggregateOptions& | Sí |  |

## Respuesta

Returns: [`AggregateResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AggregateResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de agregación'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");

AggregationRequest aggregationRequest;
aggregationRequest.setMetric(U("commentCount"));
aggregationRequest.setStartDate(U("2023-01-01T00:00:00Z"));
aggregationRequest.setEndDate(U("2023-12-31T23:59:59Z"));
aggregationRequest.setFilters({ U("status:approved") });

AggregateOptions options;
options.limit = boost::optional<int>(100);
options.includeMetadata = boost::optional<bool>(true);

api->aggregate(tenantId, aggregationRequest, options)
    .then([](pplx::task<std::shared_ptr<AggregateResponse>> task) {
        try {
            auto response = task.get();
            // procesar respuesta
        } catch (const std::exception& e) {
            // manejar error
        }
    });
[inline-code-end]

---