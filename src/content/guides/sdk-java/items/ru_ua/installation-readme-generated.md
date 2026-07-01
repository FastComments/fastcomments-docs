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

Затем добавьте нужные зависимости:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Добавьте репозиторий Repsy в файл **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.0.0"
}
```

### Содержание библиотеки

Эта библиотека содержит три модуля: сгенерированный клиент API, core‑библиотеку Java с написанными вручную утилитами, упрощающими работу с API, и модуль `pubsub`, представляющий собой библиотеку для подписки на каналы изменений.

- [Документация библиотеки API клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация core‑библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публичные и защищённые API

Для клиента API существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера/мобильного устройства и т.п. без аутентификации.

`ModerationApi` предоставляет обширный набор живых и быстрых API модерации. Каждый метод `ModerationApi` принимает параметр `sso` и может выполнять аутентификацию через SSO или через cookie‑сеанс FastComments.com.