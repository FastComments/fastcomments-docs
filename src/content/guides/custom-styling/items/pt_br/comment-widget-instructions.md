## Como Personalizar os Estilos do Widget de Comentários

Você pode personalizar a aparência do widget de comentários de duas maneiras:

### Opção 1: Via parâmetro `customCSS`

Passe seu CSS personalizado como uma string para o parâmetro `customCSS` ao inicializar o widget:

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
});
```

### Opção 2: Via Painel de Administração

1. Vá para a [Página de Personalização do Widget](https://fastcomments.com/auth/my-account/customize-widget) no seu painel de administração
2. Role até a seção "CSS Personalizado" em "Avançado"
3. Digite seu CSS personalizado
4. Clique em "Salvar"

Seu CSS personalizado será aplicado a todos os widgets de comentários no seu site.

## Dicas

- Use `!important` para sobrescrever estilos padrão, se necessário
- Mire em seletores específicos para evitar afetar outras partes do seu site
- Teste seu CSS em diferentes navegadores para compatibilidade
- O widget usa CSS padrão — nenhum pré-processador especial é necessário