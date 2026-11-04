Когато потребителските профили са отворени в контекста на вашия сайт (чрез коментарния уиджет), всички персонализирани CSS стилове, които сте приложили към вашия FastComments уиджет, се инжектират автоматично в модалния прозорец на профила.

### Как работи

Когато потребител кликне върху връзка към профил от вашия коментарен уиджет, се отваря модален прозорец на профила с класа `.fast-comments-profile`. Персонализираният CSS на вашия уиджет се инжектира автоматично в изгледа на профила. Ако вече сте стилизирали вашия коментарен уиджет, тези стилове ще се приложат и към профилите.

### CSS класове

Профилите на FastComments използват CSS архитектура, базирана на класове. Не използват CSS custom properties.

Основната страница на профила използва `.user-profile` като коренен контейнер. Хедър секцията е `.profile-header` с `.profile-header-background` за фоновото изображение. Съдържанието на профила се намира в `.profile-content`.

Аватарът използва `.profile-avatar` и `.profile-avatar-wrapper`. Името на потребителя е `.profile-name`, а текстът в биографията е `.profile-bio`. Статистиките са в `.profile-stats`, като отделните статистики използват `.stat`.

Социалните връзки са в `.profile-social-links`, като отделните връзки са `.social-link`. Значките използват `.profile-badges` и `.badge`. Лентите за напредък на значките използват `.progress-outer` и `.progress-bar`.

Табовете използват `.profile-tabs` за контейнера, `.tab` за отделните табове и `.tab.active` за избрания таб. Съдържанието на табовете използва `.tab-body` и `.tab-body.active`. Броячите на известията на табовете използват `.tab .count`.

Известията използват `.notification`, а разговорите в директни съобщения — `.conversation`. Статусът „онлайн“ е `.activity-indicator` с `.activity-indicator.online` за активното състояние. Броячите за непрочетени използват `.unread-count`.

Контейнерът на модалния прозорец на профила е `.fast-comments-profile` с `.fast-comments-profile-close` за бутона за затваряне.

### Тъмен режим

Тъмният режим използва модификатора с клас `.dark` върху `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Примери

**Хедър:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Значки:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Табове:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Модален прозорец:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```