As recargas são cobradas no método de pagamento salvo na sua conta. Se você ainda não salvou um cartão, a página de Créditos mostra `Add a payment method first` com um botão `Go to billing`. Salve um cartão na página [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e depois volte.

Para adicionar crédito:

1. Abra [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) e clique em `Manage Credits`.
2. Em `Add credit`, escolha um dos valores predefinidos ou selecione `Other amount` e digite o seu próprio valor.
3. Clique no botão `Add`. Ele mostra o valor escolhido, por exemplo `Add $25.00`.
4. Se o seu banco pedir para confirmar o pagamento, conclua a verificação na janela que se abre.

Quando o pagamento for aprovado, a página mostra `Credit added` com seu novo saldo, e a recarga aparece em `Past top-ups` como `Credited`. Clique em `Add more` para recarregar novamente.

### Valores e Limites

Os valores predefinidos são `10`, `25`, `50` e `100` na sua moeda de faturamento. Um valor personalizado pode ser qualquer valor de `5` a `10,000` por recarga, e os limites exatos para a sua moeda são exibidos abaixo do formulário. Para moedas sem casas decimais, como o iene japonês, os valores predefinidos e os limites são cem vezes maiores para representarem aproximadamente o mesmo valor.

As recargas são sempre na moeda de faturamento da sua conta, a mesma moeda das suas faturas. A página informa qual moeda será cobrada.

### Confirmação Bancária (3D Secure)

Alguns bancos exigem que você confirme um pagamento online, geralmente pelo aplicativo do banco ou por um código de uso único. Se for o seu caso, uma janela de confirmação se abre depois que você clicar em `Add`. Seu cartão só é cobrado, e seu saldo só é atualizado, depois que o banco aprova.

Se você fechar a página antes de concluir a confirmação, a recarga é registrada como `Pending`. Na próxima vez que você abrir a página de Créditos, verificamos o status do pagamento com o Stripe e o marcamos como `Credited` ou `Failed` conforme o resultado. Se o banco não aprovar o pagamento, a página mostra `Your bank did not approve the payment. Try again, or use a different card.` e nada é cobrado.

### Cartões Recusados

Se o seu cartão for recusado, a página mostra `Your card was declined. Check your card on the billing page, or try another card.` Nada é cobrado, seu saldo não muda, e a tentativa é listada em `Past top-ups` como `Failed`. Atualize o cartão na página [Billing Info](https://fastcomments.com/auth/my-account/billing-info) e tente novamente.

Para proteger o seu cartão, uma conta pode fazer um número limitado de tentativas de recarga por hora. Se você vir `Too many top-up attempts. Wait a while and try again.`, aguarde e tente novamente mais tarde.

Os pagamentos são processados pelo Stripe. A FastComments nunca armazena os dados do seu cartão.