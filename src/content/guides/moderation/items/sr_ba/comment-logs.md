FastComments аутоматски прати детаљне догађаје за сваки коментар како би омогућио транспарентност у одлукама модерације и активностима система. Ови записи помажу да разумете зашто је коментар одобрен, означен као спам или му је промијењен статус.

Записе коментара за појединачне коментаре можете видјети на контролној табли за модерирање коментара тако што ћете одабрати одређени коментар.

## Comment Log Events

Each comment maintains a log of events that occur during its lifecycle. Below are the types of events that are tracked:

### Anonymization Events
- **Anonymized** - Садржај коментара је обрисан и корисник означен као обрисан

### Approval Events
- **ApprovedDueToPastComment** - Коментар одобрен јер је корисник раније имао одобрене коментаре
- **ApprovedIsAdmin** - Коментар одобрен јер је корисник администратор
- **NotApprovedRequiresApproval** - Коментар захтијева ручно одобрење

### Spam Detection Events
- **IsSpam** - Коментар означен као спам од стране механизма за детекцију
- **IsSpamDueToBadWords** - Коментар означен као спам због филтра непристојних ријечи
- **IsSpamFromLLM** - Коментар означен као спам од стране AI/LLM механизма
- **IsSpamRepeatComment** - Коментар означен као спам због понављања
- **NotSpamIsOnlyImage** - Коментар није означен као спам јер садржи само слике
- **NotSpamIsOnlyReacts** - Коментар није означен као спам јер садржи само реакције
- **NotSpamNoLinkOrMention** - Коментар није означен као спам због недостатка сумњивих линкова или помињања
- **NotSpamPerfectTrustFactor** - Коментар није означен као спам због високог фактора повјерења корисника
- **NotSpamTooShort** - Коментар није означен као спам јер је превише кратак за анализу
- **NotSpamSkipped** - Провјера спама је прескочена
- **NotSpamFromEngine** - Механизам за детекцију утврдио да коментар није спам

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - Провјера филтра непристојних ријечи наишла је на грешку
- **BadWordsFoundBadPhrase** - Филтер непристојних ријечи открио је неприкладну фразу
- **BadWordsFoundBadWord** - Филтер непристојних ријечи открио је неприкладну ријеч
- **BadWordsNoDefinitionForLocale** - Нема дефиниција непристојних ријечи за језик коментара

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Коментар захтијева верификацију али корисник није у верификованој сесији
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Коментар захтијева верификацију али корисник још није верификован
- **InVerifiedSession** - Корисник који поставља коментар је у верификованој сесији
- **SentVerificationEmailNoSession** - Е-пошта за верификацију послана неверификованом кориснику
- **SentWelcomeEmail** - Е-пошта добродошлице послата новом кориснику

### Trust and Security Events
- **TrustFactorChanged** - Фактор повјерења корисника је промијењен
- **SpamFilterDisabledBecauseAdmin** - Филтер за спам је заобиђен јер је корисник администратор
- **TenantSpamFilterDisabled** - Филтер за спам онемогућен за цијелог tenant-а
- **RepeatCommentCheckIgnored** - Провјера понављања коментара је заобиђена
- **UserIsAdmin** - Корисник идентификован као администратор
- **UserIsAdminParentTenant** - Корисник идентификован као администратор parent tenant-а
- **UserIsAdminViaSSO** - Корисник идентификован као администратор преко SSO
- **UserIsMod** - Корисник идентификован као модератор

### Comment Status Changes
- **ExpireStatusChanged** - Статус истека коментара је промијењен
- **ReviewStatusChanged** - Статус прегледа коментара је промијењен
- **SpamStatusChanged** - Статус спама коментара је ажуриран
- **ApproveStatusChanged** - Статус одобрења коментара је промијењен
- **TextChanged** - Садржај текста коментара је уређен
- **VotesChanged** - Број гласова за коментар је ажуриран
- **Flagged** - Коментар је пријављен од корисника
- **UnFlagged** - Пријаве коментара су уклоњене

### Moderation Actions
- **Pinned** - Коментар је прикачен од стране модератора
- **UnPinned** - Коментар је одкачен од стране модератора
- **RestoredFromAnonymized** - Коментар је враћен из анонимизованог стања

### Notification Events
- **CreatedNotifications** - Обавијести су креиране за коментар
- **NotificationCreateFailure** - Неуспјело креирање обавијести
- **BadgeAwarded** - Кориснику је додјељена значка за коментар

### Publishing Events
- **PublishedLive** - Коментар је објављен уживо претплатницима

### Integration Events
- **WebhookSynced** - Коментар је синхронизован преко webhook-а

### Spam Rule Events
- **SpamRuleMatch** - Коментар је одговарао прилагођеном правилу за спам

## Accessing Comment Logs

Записи коментара се аутоматски генеришу и чувају уз сваки коментар. Они пружају вриједне увиде за:

- Разумијевање одлука модерације
- Дијагностику проблема са одобрењем/спамом
- Праћење образаца понашања корисника
- Ревизију радњи система

Ови записи помажу одржавању транспарентности у процесу модерације и помажу у прецизном подешавању понашања вашег система коментара.