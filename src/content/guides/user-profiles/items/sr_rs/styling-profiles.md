Када се профили корисника отворе у контексту вашег сајта (путем видгета за коментаре), било који прилагођени CSS стилови које сте применили на ваш FastComments видгет аутоматски се убацују у модал профила.

### Како функционише

Када корисник кликне на линк профила из вашег видгета за коментаре, отвори се модал профила са класом `.fast-comments-profile`. Ваши прилагођени CSS стилови видгета се аутоматски убацују у приказ профила. Ако сте већ стилизовали ваш коментарски видгет, ти стилови ће се примењивати и на профиле.

### CSS класе

FastComments профили користе архитектуру CSS засновану на класама. Не користи прилагођене CSS променљиве.

Главна страница профила користи `.user-profile` као коренски контејнер. Секција заглавља је `.profile-header` са `.profile-header-background` за позадинску слику. Садржај профила се налази у `.profile-content`.

Аватар користи `.profile-avatar` и `.profile-avatar-wrapper`. Име корисника је `.profile-name` а текст биографије је `.profile-bio`. Статистике су у `.profile-stats` са појединачним статистикама које користе `.stat`.

Друштвени линкови су у `.profile-social-links` са појединачним линковима као `.social-link`. Значке користе `.profile-badges` и `.badge`. Прогрес траке значки користе `.progress-outer` и `.progress-bar`.

Табови користе `.profile-tabs` за контејнер, `.tab` за појединачне табове, и `.tab.active` за изабрани таб. Садржај табова користи `.tab-body` и `.tab-body.active`. Број уведомљења на табовима користи `.tab .count`.

Уведомљења користе `.notification` а приватне поруке (DM) користе `.conversation`. Статус онлајна је `.activity-indicator` са `.activity-indicator.online` за активно стање. Нечитани бројачи користе `.unread-count`.

Контенер модал профила је `.fast-comments-profile` са `.fast-comments-profile-close` за дугме затварања.

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

**Табови:**
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