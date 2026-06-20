### Maven

Добавьте репозиторий Repsy в POM вашего проекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затем добавьте необходимые зависимости:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Добавьте репозиторий Repsy в файл build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Содержимое библиотеки

Эта библиотека содержит три модуля. Сгенерированный API-клиент, основная Java-библиотека, которая содержит вручную написанные утилиты для упрощения работы с API, и модуль `pubsub`, который является библиотекой для подписки на ленты изменений.

- [Документация библиотеки API-клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публичные и защищённые API

В API-клиенте есть три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, которые требуют ваш API-ключ, а `PublicApi` содержит методы, которые можно вызывать непосредственно из браузера/мобильного устройства и т.п. без аутентификации.

`ModerationApi` отвечает за панель модератора. Он содержит методы для модерации комментариев (просмотр списка, подсчёт, поиск, журналы и экспорт), действий модерации (удаление/восстановление, пометка, установка статусов на проверку/спам/одобрение, голоса и повторное открытие/закрытие треда), блокировок (запрет на комментирование, отмена бана, сводки перед баном, статус и настройки бана, и количество заблокированных пользователей), а также значков и доверия (назначение/удаление значка, ручные значки, получение/установка коэффициента доверия и внутренний профиль пользователя). Каждый метод `ModerationApi` принимает параметр `sso`, чтобы вызов можно было выполнить от имени модератора, аутентифицированного через SSO.