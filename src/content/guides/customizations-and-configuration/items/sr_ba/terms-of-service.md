FastComments вам омогућава да захтијевате од корисника који коментаришу први пут да прихвате ваше Услове коришћења прије него што пошаљу коментар.

Када је омогућено:
- **Анoнимни корисници** ће видјети поље за прихватање TOS-а сваки пут када коментаришу
- **Пријављени корисници** ће видјети поље само при првом коментару, или када ажурирате ваш TOS

### Омогућавање Услова коришћења

Идите на страницу за прилагођавање видџета и омогућите поље за потврду "Require Terms of Service acceptance":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Прилагођавање текста Услова коришћења

По подразумеваној вриједности, поље за потврду приказује "I agree to the Terms of Service and Privacy Policy" са линковима до оба документа. Можете прилагодити овај текст по локалу ако је потребно:

1. Одаберите "Customize text per locale"
2. Одаберите локал из падајућег менија и унесите свој прилагођени текст

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Ажурирање ваших Услова коришћења

Када ажурирате ваше Услове коришћења, подесите датум "Last Updated". Корисници који су прихватили TOS прије тог датума ће морати поново да прихвате:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Како функционише

- Временска ознака прихватања TOS-а се чува по кориснику и по коментару
- Када корисник прихвати TOS, датум се забележи у њиховом корисничком профилу (per-tenant)
- Ако поставите датум "Last Updated" који је након датума прихвата корисника, они ће морати поново да прихвате
- За анонимне кориснике које није могуће пратити, поље за потврду се појављује при сваком слању коментара