
<h1 align="center">

![MasterHead](https://se.kmitl.ac.th/assets/se.png)

</h1>

<h1 align="center">Hi 👋, There</h1>
<h3 align="center">This is my final project for</h3>

<div align="center">

### 🦀 *Elementary System Programming Course* 🦀</div>

<h3 align="center">Software Engineering, School of Engineering</h3>
<h3 align="center">

*King Mongkut's Institute of Technology Ladkrabang* 
</h3>

<h4 align="center">

**_NOTE_**

*note that all of the thing that are in this app is just the assignment of my course so don't take it so seriously*

</h4>

<h1 align= "center">

<img src="https://media.tenor.com/g7GCc40VwecAAAAi/rafs-rafsdesign.gif" width=400></img>

</h1>

<h1 align="center">NotiCheckDown</h1>
<h3 align="center">An TUI-Application that will check the status of the website or server that you've choose and send an email when website/server down.</h3>

<h1></h1>

## Installation
you need to sign up [sendgrid](https://sendgrid.com) email's api service first and then put your information on [src/helpers.rs](https://github.com/serayutaka/TUI-App/blob/main/src/helpers.rs#L34)
```
SENDGRID_API_KEY = your sendgrid's api key
SENDGRID_NAME = your name
SENDGRID_EMAIL = your email
```

## Description
NotiCheckDown provides a simple, terminal-based interface (TUI) for tracking and compiling response time data from user-specified websites at regular five-minute intervals. This application structures the data into an organised object and then converts it to a.csv file for easy data management. Furthermore, it streamlines the gathered data into an HTML email template, making it easily digestible for review.

Furthermore, NotiCheckDown converts all gathered data into a structured JSON data representation. The application seamlessly transmits this formatted data by leveraging the powerful functionality of SendGrid's email API service. Users benefit from the app's customizable notification settings, which enable automated email alerts in the event of a website or server outage.

NotiCheckDown serves as a versatile and essential tool for both monitoring and analyzing website performance, offering robust insights through regular data collection, enabling swift alerts during instances of service interruptions.

## How to use?
Simply just type this command on your terminal
```
cargo run
```
![terminal](https://github.com/serayutaka/TUI-App/assets/121752252/97416298-8bb4-457b-af45-50842746d194)

---
![result](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExdmFpMjBtZXgwZnlrYTluZGswZ29pNHI0ZnZrdDVucTBsMnVzNzFvMCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/ITvW1NqvbwXJllpjAD/giphy.gif)


- Press `h`
    - change pages to a homepage(welcome page)

- Press `c`
    - change pages to a main page
    - By changing to this page an application will forces you to put information and will close it automatically when success to send an email.

- Press `a`
    - change pages to about page

- Press `q`
    - Quit an application

<h4>

_Note that: This application will run successfully only when user not terminate a terminal_

</h4>
