---
Когда Профили пользователей открываются в контексте вашего сайта (через виджет комментариев), любые кастомные стили CSS, которые вы применили к вашему виджету FastComments, автоматически внедряются в модальное окно профиля.

### Как это работает

Когда пользователь кликает по ссылке профиля в вашем виджете комментариев, открывается модальное окно профиля с классом `.fast-comments-profile`. Кастомные CSS-стили вашего виджета автоматически внедряются в представление профиля. Если вы уже стилизовали ваш виджет комментариев, эти стили будут применяться и к профилям.

### Классы CSS

Профили FastComments используют архитектуру CSS, основанную на классах. Они не используют CSS-переменные.

Главная страница профиля использует `.user-profile` в качестве корневого контейнера. Секция заголовка — `.profile-header` с `.profile-header-background` для фонового изображения. Контент профиля находится в `.profile-content`.

Аватар использует `.profile-avatar` и `.profile-avatar-wrapper`. Имя пользователя — `.profile-name`, а текст биографии — `.profile-bio`. Статистика находится в `.profile-stats`, а отдельные показатели используют `.stat`.

Социальные ссылки находятся в `.profile-social-links`, а отдельные ссылки — `.social-link`. Значки используют `.profile-badges` и `.badge`. Полосы прогресса значков используют `.progress-outer` и `.progress-bar`.

Вкладки используют `.profile-tabs` для контейнера, `.tab` для отдельных вкладок и `.tab.active` для выбранной вкладки. Контент вкладок использует `.tab-body` и `.tab-body.active`. Счётчики уведомлений на вкладках используют `.tab .count`.

Уведомления используют `.notification`, а приватные сообщения — `.conversation`. Индикатор онлайн-статуса — `.activity-indicator` с `.activity-indicator.online` для активного состояния. Счётчики непрочитанных используют `.unread-count`.

Контейнер модального окна профиля — `.fast-comments-profile` с `.fast-comments-profile-close` для кнопки закрытия.

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

---