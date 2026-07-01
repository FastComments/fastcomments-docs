### Maven

Додайте репозиторій Repsy у файл POM вашого проекту:

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

Додайте репозиторій Repsy у файл **build.gradle**:

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

Ця бібліотека містить три модулі. Згенерований клієнт API, ядрову Java‑бібліотеку, яка містить написані вручну утиліти
для спрощення роботи з API, та модуль `pubsub`, який є бібліотекою для підписки на потоки змін.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Для клієнта API існують три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API‑ключ, а `PublicApi` містить методи,
які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.

`ModerationApi` надає широкий набір живих і швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie‑сесію FastComments.com.