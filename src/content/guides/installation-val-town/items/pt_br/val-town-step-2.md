`tenantId: "demo"` é um sandbox público compartilhado. Ele funciona sem cadastro, por isso os exemplos o utilizam, mas todos os outros que experimentam o FastComments escrevem nas mesmas discussões e qualquer pessoa pode moderá‑las. Troque antes de publicar algo que lhe importe.

Seu ID de locatário está na [página de segredo da API](https://fastcomments.com/auth/my-account/api-secret).

Um ID de locatário é público e deve estar no código do navegador. Um segredo de API não é, e nada nesta página requer um.

## Leia-a a partir de uma variável de ambiente

Os vals do Val Town são públicos no plano gratuito, portanto sua origem é legível por todos. Mantenha tudo que for sensível em variáveis de ambiente, lidas com `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Isso é ainda mais importante no Val Town por um segundo motivo: **remixar um val copia as chaves das variáveis de ambiente, mas não seus valores.** Um segredo mantido em uma variável de ambiente não acompanha seu val para a conta de outra pessoa. Um segredo escrito em um arquivo acompanha.

Recuar para `"demo"` mantém o val funcionando para quem o remixa antes de definir seu próprio locatário.

## Contas da UE

Uma conta, seus dados e suas chaves residem em uma única região. Se a sua foi criada em `eu.fastcomments.com`, toda configuração de widget também precisa de `region: "eu"`, e os scripts são carregados de `cdn-eu.fastcomments.com`. Caso contrário, deixe ambos como estão.

---