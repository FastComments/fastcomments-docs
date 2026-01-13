Додайте FastComments SDK у файл `build.gradle.kts` вашого додатку:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Переконайтеся, що у вашому проєкті є репозиторій Repsy у файлі `settings.gradle.kts`:

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