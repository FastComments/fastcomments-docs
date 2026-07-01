### Maven

Add the Repsy repository to your project's POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

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

Add the Repsy repository to your build.gradle file:

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

### Library Contents

Эта библиотека содержит три модуля: сгенерированный клиент API, основную Java‑библиотеку, которая содержит написанные вручную утилиты для упрощения работы с API, и модуль `pubsub`, который представляет собой библиотеку для подписки на потоки изменений.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация основной библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Для клиентской библиотеки API существует три класса: `DefaultApi`, `PublicApi` и `ModerationApi`. Класс `DefaultApi` содержит методы, требующие вашего API‑ключа, а `PublicApi` — методы, которые можно вызывать напрямую из браузера, мобильного устройства и т.д. без аутентификации.

`ModerationApi` предоставляет обширный набор живых и быстрых модерационных API. Каждый метод `ModerationApi` принимает параметр `sso` и может аутентифицироваться через SSO или с помощью cookies сеанса FastComments.com.