Após chamar `sdk.load()`, o SDK inscreve-se automaticamente em eventos WebSocket para o `urlId` configurado. Os seguintes eventos são tratados:

- Novos comentários, edições e exclusões
- Votos (novos e removidos)
- Alterações de estado de pin, lock, flag e block
- Presença do usuário (entrada/saída)
- Abertura/fechamento de thread
- Concessões de badges
- Atualizações de configuração do servidor

### Controlando a exibição ao vivo

Por padrão, novos comentários de outros usuários aparecem imediatamente:

```swift
sdk.showLiveRightAway = true   // Padrão: mostrar instantaneamente
```

Defina isso como `false` para armazenar novos comentários atrás de um botão 'N novos comentários', permitindo que o usuário escolha quando revelá-los:

```swift
sdk.showLiveRightAway = false
```

### Presença do usuário

Indicadores online/offline aparecem automaticamente nos avatares dos usuários quando o servidor habilita o rastreamento de presença. Nenhuma configuração adicional é necessária no cliente.

---
---