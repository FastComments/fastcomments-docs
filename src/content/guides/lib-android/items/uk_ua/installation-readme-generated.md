Додайте FastComments SDK до файлу `build.gradle.kts` вашого застосунку:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Переконайтеся, що у вашому проєкті в `settings.gradle.kts` наявний репозиторій Repsy:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // інші репозиторії...
    }
}
```