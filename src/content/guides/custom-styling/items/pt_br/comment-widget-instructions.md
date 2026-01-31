## Como personalizar os estilos do widget de comentários

Você pode personalizar o estilo do widget de comentários de duas maneiras:

### Opção 1: Via parâmetro customCSS

Passe seu CSS personalizado como uma string para o parâmetro `customCSS` ao inicializar o widget:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Opção 2: Via Painel de Administração

1. Acesse a [página de Personalização do Widget](https://fastcomments.com/auth/my-account/customize-widget) no seu painel de administração
2. Role até a seção "CSS personalizado" em "Avançado"
3. Insira seu CSS personalizado
4. Clique em "Salvar"

Seu CSS personalizado será aplicado a todos os widgets de comentários no seu site.

## Dicas

- Use `!important` para sobrescrever os estilos padrão, se necessário
- Use seletores específicos para evitar afetar outras partes do seu site
- Teste seu CSS em diferentes navegadores para compatibilidade
- O widget usa CSS padrão - nenhum pré-processador especial é necessário