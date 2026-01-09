---
Um objeto `EmailTemplate` representa a configuração para um modelo de e-mail personalizado, para um tenant.

O sistema selecionará o modelo de e-mail a ser usado via:

- Seu identificador de tipo, chamamos isto de `emailTemplateId`. Estes são constantes.
- O `domain`. Primeiro tentaremos encontrar um template para o domínio ao qual o objeto relacionado (como um `Comment`) está vinculado, e se não for encontrada uma correspondência então tentaremos encontrar um template onde domain seja null ou `*`.

A estrutura para o objeto `EmailTemplate` é a seguinte:

[inline-code-attrs-start title = 'Estrutura do Modelo de Email'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SOMENTE LEITURA **/
    createdAt: string
    /** SOMENTE LEITURA **/
    updatedAt: string
    /** SOMENTE LEITURA **/
    updatedByUserId: string
    /** O domínio ao qual o modelo deve estar associado. **/
    domain?: string | '*' | null
    /** O conteúdo do modelo de e-mail em sintaxe EJS. **/
    ejs: string
    /** Um mapa de chaves de tradução sobrescritas para valores, para cada localidade suportada. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Um objeto que representa o contexto de renderização do modelo. **/
    testData: object
}
[inline-code-end]

### Notas

- Você pode obter os valores válidos de `emailTemplateId` a partir do endpoint `/definitions`.
- O endpoint `/definitions` também inclui as traduções padrão e os dados de teste.
- Os modelos não serão salvos se a estrutura ou os dados de teste forem inválidos.

---