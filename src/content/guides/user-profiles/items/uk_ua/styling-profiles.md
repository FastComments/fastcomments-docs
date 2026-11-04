Коли профілі користувачів відкриваються в контексті вашого сайту (через віджет коментарів), будь-які власні CSS-стилі, які ви застосували до вашого віджета FastComments, автоматично вставляються в модальне вікно профілю.

### Як це працює

Коли користувач натискає на посилання профілю у вашому віджеті коментарів, відкривається модальне вікно профілю з класом `.fast-comments-profile`. Власний CSS вашого віджета автоматично вставляється в перегляд профілю. Якщо ви вже стилізували свій віджет коментарів, ці стилі застосуються до профілів.

### CSS-класи

Профілі FastComments використовують CSS-архітектуру на основі класів. Вони не використовують CSS custom properties.

Головна сторінка профілю використовує `.user-profile` як кореневий контейнер. Секція заголовка — це `.profile-header` з `.profile-header-background` для фонового зображення. Вміст профілю розміщено в `.profile-content`.

Аватар використовує `.profile-avatar` та `.profile-avatar-wrapper`. Ім'я користувача — `.profile-name`, а текст біографії — `.profile-bio`. Статистика знаходиться в `.profile-stats`, окремі показники використовують `.stat`.

Соціальні посилання знаходяться в `.profile-social-links`, окремі посилання — `.social-link`. Значки використовують `.profile-badges` та `.badge`. Індикатори прогресу значків використовують `.progress-outer` та `.progress-bar`.

Вкладки використовують `.profile-tabs` для контейнера, `.tab` для окремих вкладок та `.tab.active` для вибраної вкладки. Вміст вкладок використовує `.tab-body` та `.tab-body.active`. Лічильники сповіщень на вкладках використовують `.tab .count`.

Сповіщення використовують `.notification`, а бесіди в особистих повідомленнях — `.conversation`. Статус онлайн — це `.activity-indicator` з `.activity-indicator.online` для активного стану. Лічильники непрочитаного використовують `.unread-count`.

Контейнер модального вікна профілю — це `.fast-comments-profile` з `.fast-comments-profile-close` для кнопки закриття.

### Темний режим

Темний режим використовує модифікатор класу `.dark` на `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Приклади

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

**Модальне вікно:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```