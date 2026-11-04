### Maven

Додайте репозиторій Repsy до POM-файлу вашого проєкту:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Потім додайте потрібні залежності:

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

Додайте репозиторій Repsy до файлу build.gradle:

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

### Вміст бібліотеки

Ця бібліотека містить три модулі: згенерований API-клієнт, основну Java-бібліотеку з написаними вручну утилітами для спрощення роботи з API, а також модуль `pubsub` — бібліотеку для підписки на стрічки змін.

- [Документація бібліотеки API-клієнта](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документація основної бібліотеки, включно з прикладами SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документація бібліотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публічні та захищені API

В API-клієнті є три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які потребують ваш API-ключ, а `PublicApi` — методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.

`ModerationApi` надає розширений набір швидких API для модерації в реальному часі. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.