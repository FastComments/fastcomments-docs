## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Respuesta

Devuelve: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_gifs_trending_response.rb)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de get_gifs_trending'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Cadena | 
opts = {
  locale: 'locale_example', # Cadena | 
  rating: 'rating_example', # Cadena | 
  page: 1.2 # Flotante | 
}

begin
  
  result = api_instance.get_gifs_trending(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_gifs_trending: #{e}"
end
[inline-code-end]

---