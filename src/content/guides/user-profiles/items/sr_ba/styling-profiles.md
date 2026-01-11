Када се кориснички профили отворе у контексту ваше странице (путем видгета за коментаре), све прилагођене CSS стилове које сте применили на ваш FastComments видгет аутоматски се убацују у модал профила.

### Како то функционише

Када корисник кликне на линк профила из вашег видгета за коментаре, отвара се модал профила са класом `.fast-comments-profile`. Ваш прилагођени CSS видгета аутоматски се убацује у приказ профила. Ако сте већ обликовали ваш видгет за коментаре, ти стилови ће важити и за профиле.

### CSS класе

FastComments профили користе архитектуру CSS засновану на класама. Не користи CSS прилагођена својства.

Главна страница профила користи `.user-profile` као коренски контејнер. Секција заглавља је `.profile-header` са `.profile-header-background` за позадинску слику. Садржај профила се налази у `.profile-content`.

Аватар користи `.profile-avatar` и `.profile-avatar-wrapper`. Име корисника је `.profile-name` а биографски текст је `.profile-bio`. Статистике су у `.profile-stats` са појединачним статистикама које користе `.stat`.

Друштвени линкови су у `.profile-social-links` са појединачним линковима као `.social-link`. Значке користе `.profile-badges` и `.badge`. Траке напретка значки користе `.progress-outer` и `.progress-bar`.

Картице користе `.profile-tabs` за контејнер, `.tab` за појединачне картице, и `.tab.active` за изабрану картицу. Садржај картице користи `.tab-body` и `.tab-body.active`. Бројачи обавештења на картицама користе `.tab .count`.

Обавештења користе `.notification` а приватни разговори користе `.conversation`. Онлајн статус је `.activity-indicator` са `.activity-indicator.online` за активно стање. Бројачи непрочитаних користе `.unread-count`.

Модал контејнер профила је `.fast-comments-profile` са `.fast-comments-profile-close` за дугме затварања.

### Тамни режим

Тамни режим користи модификатор класе `.dark` на `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Примери

**Заглавље:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Значке:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Картице:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Модал:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```