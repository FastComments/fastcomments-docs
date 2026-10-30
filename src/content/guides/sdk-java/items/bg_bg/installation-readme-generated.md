### Maven

Добавете репозитория Repsy към POM на вашия проект:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

След това добавете необходимите зависимости:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Добавете репозитория Repsy към вашия файл **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:3.1.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Съдържание на библиотеката

Тази библиотека съдържа три модула: генерирания API клиент, основната Java библиотека, която включва ръчно написани помощни функции за по‑лесна работа с API, и модула `pubsub`, който е библиотека за абониране към потоци от промени.

- [Документация за библиотеката на API клиента](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документация за основната библиотека, включително примери за SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документация за PubSub библиотеката](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публични vs Защитени API

За API клиента има три класа – `DefaultApi`, `PublicApi` и `ModerationApi`. `DefaultApi` съдържа методи, които изискват вашия API ключ, докато `PublicApi` съдържа методи, които могат да се извикат директно от браузър/мобилно устройство и т.н. без удостоверяване.

`ModerationApi` предоставя обширен набор от живи и бързи API за модериране. Всеки метод на `ModerationApi` приема параметър `sso` и може да се удостоверява чрез SSO или чрез сесийна бисквитка от FastComments.com.