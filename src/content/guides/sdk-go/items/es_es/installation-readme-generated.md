```bash
go get github.com/fastcomments/fastcomments-go
```

### Uso del cliente API

#### API pública (Sin autenticación)

La PublicAPI permite acceso no autenticado a endpoints públicos:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // Obtener comentarios con PublicAPI
    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### API predeterminada (Requiere clave API)

La DefaultAPI requiere autenticación mediante tu clave API:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // Crear contexto autenticado con la clave API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Obtener comentarios usando la DefaultAPI autenticada
    response, httpResp, err := apiClient.DefaultAPI.GetComments(auth).
        TenantId("your-tenant-id").
        UrlId("your-page-url-id").
        Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### API de moderación (Panel de moderador)

La ModerationAPI alimenta el panel de moderador. Proporciona métodos para listar,
contar, buscar y exportar comentarios, acciones de moderación (eliminar/restaurar,
marcar, establecer estado de revisión/spam/aprobación, votos, reabrir/cerrar hilos), baneos (prohibir comentar, deshacer, resúmenes previos al baneo, estado y preferencias de baneo, recuentos de usuarios baneados),
e insignias y confianza (otorgar/quitar insignias, insignias manuales, obtener/establecer factor de confianza, perfil interno de usuario). Todos los métodos de Moderation aceptan un parámetro `sso` para
moderadores autenticados por SSO:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // Listar comentarios para moderación usando ModerationAPI
    response, httpResp, err := apiClient.ModerationAPI.GetApiComments(
        context.Background(),
    ).Sso("your-sso-token").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```