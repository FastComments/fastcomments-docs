---
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
    <!-- Клиент API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Базовая библиотека (включает SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Библиотека PubSub (для событий в реальном времени) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // Клиент API
    implementation "com.fastcomments:client:1.3.2"
    
    // Базовая библиотека (включает SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Библиотека PubSub (для событий в реальном времени)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Library Contents

Эта библиотека содержит три модуля. Сгенерированный клиент API, базовая Java-библиотека, содержащая вручную написанные утилиты для упрощения работы с API, и модуль `pubsub`, который является библиотекой для подписки на потоки изменений.

- [Документация клиентской библиотеки API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация базовой библиотеки, включая примеры SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация библиотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Для клиента API существуют два класса, `DefaultApi` и `PublicApi`. `DefaultApi` содержит методы, требующие вашего API-ключа, а `PublicApi` содержит вызовы API, которые можно выполнять напрямую из браузера/мобильного устройства и т.д. без аутентификации.
---