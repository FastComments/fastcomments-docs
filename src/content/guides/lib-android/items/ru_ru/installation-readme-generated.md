Добавьте SDK FastComments в файл `build.gradle.kts` вашего приложения:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Убедитесь, что репозиторий Repsy указан в `settings.gradle.kts` вашего проекта:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```