---
Quando Perfis de Usuário são abertos no contexto do seu site (via o widget de comentários), quaisquer estilos CSS personalizados que você aplicou ao seu widget FastComments são automaticamente injetados no modal de perfil.

### Como Funciona

Quando um usuário clica em um link de perfil no seu widget de comentários, um modal de perfil é aberto com a classe `.fast-comments-profile`. O CSS personalizado do seu widget é automaticamente injetado na visualização do perfil. Se você já estilizou seu widget de comentários, esses estilos serão aplicados aos perfis.

### Classes CSS

Perfis do FastComments usam uma arquitetura CSS baseada em classes. Não utiliza propriedades personalizadas do CSS.

A página principal do perfil usa `.user-profile` como o contêiner raiz. A seção de cabeçalho é `.profile-header` com `.profile-header-background` para a imagem de fundo. O conteúdo do perfil fica em `.profile-content`.

O avatar usa `.profile-avatar` e `.profile-avatar-wrapper`. O nome do usuário é `.profile-name` e o texto da biografia é `.profile-bio`. As estatísticas estão em `.profile-stats` com estatísticas individuais usando `.stat`.

Links sociais estão em `.profile-social-links` com links individuais como `.social-link`. Insígnias usam `.profile-badges` e `.badge`. Barras de progresso das insígnias usam `.progress-outer` e `.progress-bar`.

Abas usam `.profile-tabs` como contêiner, `.tab` para abas individuais, e `.tab.active` para a aba selecionada. O conteúdo das abas usa `.tab-body` e `.tab-body.active`. Contadores de notificações nas abas usam `.tab .count`.

Notificações usam `.notification` e conversas de mensagens diretas usam `.conversation`. O status online é `.activity-indicator` com `.activity-indicator.online` para o estado ativo. Contadores de não lidos usam `.unread-count`.

O contêiner do modal de perfil é `.fast-comments-profile` com `.fast-comments-profile-close` para o botão de fechar.

### Modo Escuro

Dark mode uses the `.dark` class modifier on `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Exemplos

**Cabeçalho:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Insígnias:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Abas:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```

---