Когато Потребителските профили се отварят в контекста на вашия сайт (чрез коментарния уиджет), всички персонализирани CSS стилове, които сте приложили към FastComments уиджета, се внедряват автоматично в модалния прозорец на профила.

### How It Works

Когато потребител кликне върху връзка към профил от вашия коментарен уиджет, се отваря модален прозорец на профила с класа `.fast-comments-profile`. Персонализираният CSS на уиджета ви се вкарва автоматично в изгледа на профила. Ако вече сте стилували коментарния си уиджет, тези стилове ще се прилагат и към профилите.

### CSS Classes

Профилите на FastComments използват CSS архитектура, базирана на класове. Те не използват CSS custom properties.

Основната страница на профила използва `.user-profile` като коренов контейнер. Секцията на заглавката е `.profile-header` с `.profile-header-background` за фоновото изображение. Съдържанието на профила се намира в `.profile-content`.

Аватарът използва `.profile-avatar` и `.profile-avatar-wrapper`. Името на потребителя е `.profile-name`, а биографията е `.profile-bio`. Статистиките са в `.profile-stats`, като отделните статистики използват `.stat`.

Социалните връзки са в `.profile-social-links`, а отделните връзки са `.social-link`. Значките използват `.profile-badges` и `.badge`. Лентовете за напредък на значките използват `.progress-outer` и `.progress-bar`.

Табовете използват `.profile-tabs` за контейнера, `.tab` за отделните табове и `.tab.active` за избрания таб. Съдържанието на табовете използва `.tab-body` и `.tab-body.active`. Броячите за известия върху табовете използват `.tab .count`.

Известията използват `.notification`, а разговорите в лични съобщения (DM) използват `.conversation`. Онлайн статусът е `.activity-indicator` с `.activity-indicator.online` за активното състояние. Нечетените броячи използват `.unread-count`.

Контейнерът на модалния прозорец на профила е `.fast-comments-profile`, а бутона за затваряне е `.fast-comments-profile-close`.

### Dark Mode

Тъмният режим използва модификатора на класа `.dark` върху `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Examples

**Header:**
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

**Модал:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```