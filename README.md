# BambangShop Publisher App

Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project

In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:

1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment

1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable | type | description |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)

- [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
- **STAGE 1: Implement models and repositories**
  - [x] Commit: `Create Subscriber model struct.`
  - [x] Commit: `Create Notification model struct.`
  - [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
  - [x] Commit: `Implement add function in Subscriber repository.`
  - [x] Commit: `Implement list_all function in Subscriber repository.`
  - [x] Commit: `Implement delete function in Subscriber repository.`
  - [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
- **STAGE 2: Implement services and controllers**
  - [x] Commit: `Create Notification service struct skeleton.`
  - [x] Commit: `Implement subscribe function in Notification service.`
  - [x] Commit: `Implement subscribe function in Notification controller.`
  - [x] Commit: `Implement unsubscribe function in Notification service.`
  - [x] Commit: `Implement unsubscribe function in Notification controller.`
  - [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
- **STAGE 3: Implement notification mechanism**
  - [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
  - [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
  - [ ] Commit: `Implement publish function in Program service and Program controller.`
  - [ ] Commit: `Edit Product service methods to call notify after create/delete.`
  - [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections

This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

> 1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

Karena pada kasus BambangShop ini hanya terdapat satu jenis Subscriber, maka cukup menggunakan satu struct tanpa perlu trait karena lebih sederhana dan tidak ada kebutuhan untuk fleksibilitas. Namun, jika ada beberapa jenis subscriber yang perlu merespons notifikasi dengan cara berbeda (misalnya pelanggan, admin, atau supplier), maka penggunaan trait seperti dalam pola desain Observer sangat berguna untuk mendukung polimorfisme dan prinsip inversi ketergantungan (DIP).

> 2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

Penggunaan DashMap lebih tepat dibandingkan Vec karena DashMap memungkinkan akses dan pencarian yang lebih cepat menggunakan kunci unik tanpa perlu melakukan iterasi seperti pada Vec. Selain itu, jika sistem perlu sering melakukan pencarian, pembaruan, atau penghapusan berdasarkan id atau url, maka DashMap lebih efisien dibandingkan dengan Vec.

> 3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

Karena SUBSCRIBERS adalah variabel statis yang digunakan secara global dan harus thread-safe, maka penggunaan DashMap tetap lebih tepat dibandingkan hanya mengandalkan pola Singleton. DashMap sudah didesain sebagai concurrent HashMap, memungkinkan multiple threads untuk membaca dan menulis tanpa perlu mengelola mutex secara manual.

#### Reflection Publisher-2

> 1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

Dalam praktik pengembangan modern, memisahkan Service dan Repository dari Model mengikuti prinsip Separation of Concerns (SoC) dan Single Responsibility Principle (SRP), yang membuat kode lebih modular, mudah diuji, dan dipelihara. Model dalam MVC klasik memang mencakup logika bisnis dan akses data, tetapi dalam skala aplikasi yang lebih besar, ini menyebabkan tight coupling dan sulitnya melakukan perubahan tanpa memengaruhi banyak bagian sistem.

> 2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

Jika kita hanya menggunakan Model tanpa memisahkan Service dan Repository, maka setiap Model akan menangani akses data dan logika bisnis secara langsung, yang menyebabkan tight coupling antar model dan meningkatnya kompleksitas kode. Misalnya, dalam konteks Program, Subscriber, dan Notification, jika Program perlu mengirim notifikasi ke semua Subscriber, maka Program harus langsung mengakses dan memproses data dari Subscriber dan Notification, membuat setiap model saling bergantung. Hal ini memperumit pemeliharaan, karena perubahan pada satu model bisa berdampak pada banyak bagian lain.

> 3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

Ya, saya sudah mencoba eksplorasi lebih lanjut terkait penggunaan Postman. Menurut saya, Postman sangat membantu untuk testing endpoint yang terdapat pada proyek sekarang dengan mempersiapkan URL dengan methodnya yang sudah jadi dan hanya perlu menekan satu tombol untuk melakukan testing terhadap suatu endpoint. Fitur inilah yang sangat benefisial terutama untuk Group Project.

#### Reflection Publisher-3
