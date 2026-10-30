### Maven

Додайте репозиторій Repsy у файл POM вашого проєкту:

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
    <!-- Клієнт API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Основна бібліотека (включає SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.1</version>
    </dependency>
    
    <!-- Бібліотека PubSub (для живих подій) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.1</version>
    </dependency>
</dependencies>
```

### Gradle

Додайте репозиторій Repsy у файл `build.gradle`:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Клієнт API
    implementation "com.fastcomments:client:3.1.1"
    
    // Основна бібліотека (включає SSO)
    implementation "com.fastcomments:core:3.1.1"
    
    // Бібліотека PubSub (для живих подій)
    implementation "com.fastcomments:pubsub:3.1.1"
}
```

### Library Contents

Ця бібліотека містить три модулі: згенерований клієнт API, основну Java‑бібліотеку, яка містить написані вручну утиліти для спрощення роботи з API, та модуль `pubsub`, який є бібліотекою для підписки на потоки змін.

- [Документація бібліотеки клієнта API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документація основної бібліотеки, включаючи приклади SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документація бібліотеки PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Для клієнта API існує три класи: `DefaultApi`, `PublicApi` та `ModerationApi`. `DefaultApi` містить методи, які вимагають ваш API‑ключ, а `PublicApi` містить методи, які можна викликати безпосередньо з браузера/мобільного пристрою тощо без автентифікації.

`ModerationApi` надає широкий набір живих та швидких API модерації. Кожен метод `ModerationApi` приймає параметр `sso` і може автентифікуватися через SSO або cookie сесії FastComments.com.