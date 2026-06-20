### Maven

Додајте Repsy репозиторијум у POM вашег пројекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затим додајте зависности које су вам потребне:

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

Додајте Repsy репозиторијум у ваш build.gradle фајл:

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

### Садржај библиотеке

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане помоћне класе да би рад са API-јем био једноставнији, и модул `pubsub` који је библиотека за претплату на фидове промена.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација основне библиотеке, укључујући примјере SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Јавни и заштићени API-ји

За API клијента постоје три класе, `DefaultApi`, `PublicApi`, и `ModerationApi`. `DefaultApi` садржи методе које захтијевају ваш API кључ, а `PublicApi` садржи методе које се могу позвати директно из прегледача/мобилног уређаја/итд. без аутентификације.

`ModerationApi` покреће контролну таблу модератора. Садржи методе за модерацију коментара (листање, бројање, претрага, логови и извоз), акције модерације (уклони/врати, означи, подеси статус за преглед/спам/одобрење, гласови, и поновно отварање/затварање теме), забране (забрана коментисања, поништавање забране, сажеци предзабране, статус и преференције забране, и бројање забрањених корисника), и значке и повјерење (додијели/уклони значку, ручне значке, добити/поставити фактор повјерења, и унутрашњи профил корисника). Сваки метод `ModerationApi` прихвата параметар `sso` тако да се позив може извршити у име модератора аутентификованог преко SSO.