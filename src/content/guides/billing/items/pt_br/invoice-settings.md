A página [Invoice Settings](https://fastcomments.com/auth/my-account/invoice-settings), acessada pelo botão `Invoice Settings` em Billing Info, controla o que aparece nas suas faturas e para onde elas são enviadas. Apenas o `Account Owner` ou um `Billing Admin` podem abri-la.

- `Company Name`: o nome da sua conta como exibido no painel, em e-mails e nos convites da equipe. Este não é o nome de cobrança.
- `Billing Email`: para onde as faturas e notificações de cobrança são enviadas. Defina isso como uma caixa de e-mail financeira ou de contas se o proprietário da conta não deve recebê-las. Isso não altera o e-mail de login de nenhum usuário.
- `Invoice Currency`: a moeda em que suas faturas são emitidas. Veja [Currencies](#currencies).
- `Billing Address`: o `Name`, `Address`, `City`, `State / Province / Region`, `Postal Code` e `Country` impressos nas faturas. Os mesmos campos aparecem no formulário Billing Info quando você salva um cartão; ambos gravam no mesmo lugar.
- `VAT / Tax ID`: opcional. Impresso nas suas faturas.

Clique em `Save Changes`. A página confirma com `Invoice settings updated successfully.` e as alterações se aplicam à próxima fatura. Faturas existentes não são alteradas.

### VAT And Tax IDs

Selecione o `Country` de cobrança antes de inserir o ID, porque o país determina que tipo de ID é: um número de IVA da UE para qualquer estado-membro da UE, um número de IVA do Reino Unido para o Reino Unido, um ABN para a Austrália, um número de GST/HST para o Canadá, um EIN para os Estados Unidos, e assim por diante para os outros países que o Stripe suporta.

O ID é validado quando você salva. Se não for válido para o país selecionado, a página informa isso e o ID não é salvo, enquanto suas outras configurações são. Se o seu país não for um para o qual podemos registrar um ID fiscal, a página também informa isso.

Se você tiver um endereço de cobrança na UE, as faturas Flex são emitidas com 0% de IVA como uma transação de cobrança reversa, e seu número de IVA aparece nelas assim que salvo, para que seu contador possa lidar com o IVA do seu lado.