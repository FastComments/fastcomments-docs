### Maven

Додайте репозиторій Repsy до POM вашого проєкту:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Потім додайте необхідні залежності:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додайте репозиторій Repsy у файл build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:0.0.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Вміст бібліотеки

Ця бібліотека містить три модулі. Згенерований клієнт API, основна бібліотека Java, яка містить вручну написані утиліти для полегшення роботи з API, та модуль `pubsub`, який є бібліотекою для підписки на потоки змін.

- [Документація клієнтської бібліотеки API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документація основної бібліотеки, включно з прикладами SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документація бібліотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Публічні та захищені API

Для клієнта API існують два класи, `DefaultApi` та `PublicApi`. `DefaultApi` містить методи, які вимагають вашого API-ключа, а `PublicApi` містить виклики API, які можна виконувати без автентифікації безпосередньо з браузера/мобільного пристрою/тощо.