Config vem de três lugares. Fontes posteriores prevalecem:

1. **Padrões globais** em `_config.yml` sob a chave `fastcomments:`.
2. **Contexto da página**, derivado automaticamente para widgets com escopo de página (veja abaixo).
3. **Atributos da tag** escritos na própria tag.

Então um `url_id` na tag sobrescreve o valor derivado da página, que por sua vez sobrescreve qualquer padrão global.

### Sintaxe de atributos

Atributos são pares `key=value` em `snake_case`:

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **Valores entre aspas** (`"..."` ou `'...'`) são strings literais.
- **Sem aspas** `true`/`false` tornam-se booleanos, e números tornam-se números.
- **Sem aspas** qualquer outra coisa é resolvida como uma variável Liquid a partir do contexto da página, por exemplo
  `url_id=page.slug`. (Liquid does not expand `{% raw %}\{{ ... }}{% endraw %}` inside a tag's attributes, so use the bare `page.slug` form rather than `"{% raw %}\{{ page.slug }}{% endraw %}"`.)

Chaves em snake_case de atributos e de configuração são mapeadas automaticamente para as chaves em camelCase que o FastComments espera (`tenant_id` → `tenantId`, `url_id` → `urlId`, `page_title` → `pageTitle`,
`has_dark_background` → `hasDarkBackground`, e assim por diante). Qualquer outra opção da
[widget configuration](https://docs.fastcomments.com/guide-customizations-and-configuration.html)
passa direto da mesma maneira.

### Valores derivados da página

Para os widgets com escopo de página (`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`,
`fastcomments_collab_chat`, `fastcomments_image_chat`) estes são preenchidos automaticamente a partir da
página atual, a menos que você os defina você mesmo:

- `url_id` ← `page.url` (um identificador estável independente do domínio visitante)
- `url` ← `site.url` + `page.url` (apenas quando `url` está definido em `_config.yml`)
- `page_title` ← `page.title`

Widgets de site inteiro (comentários/discussões recentes, páginas principais, resumo de avaliações, feed de atividade do usuário,
contagem em massa) não estão vinculados a uma página e não derivam estes valores.

### Residência de dados na UE

Clientes da UE adicionam `region: eu`, seja globalmente:

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

ou por tag: `{% raw %}{% fastcomments region="eu" %}{% endraw %}`. Os widgets então carregam a partir da CDN da UE.