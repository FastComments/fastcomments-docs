---
Додайте FastComments SDK у файл `build.gradle.kts` вашого застосунку:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Переконайтеся, що у вашому проекті `settings.gradle.kts` є репозиторій Repsy:

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
---