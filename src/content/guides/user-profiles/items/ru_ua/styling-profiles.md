Когда профили пользователей открываются в контексте вашего сайта (через виджет комментариев), любые пользовательские CSS-стили, которые вы применили к вашему виджету FastComments, автоматически внедряются в модальное окно профиля.

### Как это работает

Когда пользователь нажимает на ссылку профиля в вашем виджете комментариев, открывается модальное окно профиля с классом `.fast-comments-profile`. Пользовательский CSS вашего виджета автоматически внедряется в представление профиля. Если вы уже стилизовали виджет комментариев, эти стили будут применяться и к профилям.

### CSS-классы

Профили FastComments используют архитектуру CSS на основе классов. Они не используют кастомные CSS-переменные.

Основная страница профиля использует `.user-profile` как корневой контейнер. Секция заголовка — `.profile-header` с `.profile-header-background` для фонового изображения. Содержимое профиля располагается в `.profile-content`.

Аватар использует `.profile-avatar` и `.profile-avatar-wrapper`. Имя пользователя — `.profile-name`, а текст био — `.profile-bio`. Статистика находится в `.profile-stats`, а отдельные показатели используют `.stat`.

Социальные ссылки находятся в `.profile-social-links`, отдельные ссылки — `.social-link`. Значки используют `.profile-badges` и `.badge`. Полосы прогресса для значков используют `.progress-outer` и `.progress-bar`.

Вкладки используют `.profile-tabs` для контейнера, `.tab` для отдельных вкладок и `.tab.active` для выбранной вкладки. Содержимое вкладки использует `.tab-body` и `.tab-body.active`. Счётчики уведомлений на вкладках находятся в `.tab .count`.

Уведомления используют `.notification`, а приватные сообщения — `.conversation`. Индикатор онлайн-статуса — `.activity-indicator`, с `.activity-indicator.online` для активного состояния. Счётчики непрочитанных используют `.unread-count`.

Контейнер модального окна профиля — `.fast-comments-profile`, кнопка закрытия — `.fast-comments-profile-close`.

### Темная тема

Темная тема использует модификатор класса `.dark` на `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Примеры

**Заголовок:**
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

**Вкладки:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Модальное окно:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```