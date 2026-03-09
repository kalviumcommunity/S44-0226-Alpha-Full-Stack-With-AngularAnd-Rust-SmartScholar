import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './login.html',
  styleUrl: './login.css',
})
export class Login {
  email = '';
  password = '';

  login() {
    if (!this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    console.log('Login attempt:', this.email);
  }
}
