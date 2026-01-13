---
Uygulamanızın `build.gradle.kts` dosyasına FastComments SDK'sını ekleyin:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Projenizin `settings.gradle.kts` dosyasında Repsy deposunun bulunduğundan emin olun:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // diğer depolar...
    }
}
```
---