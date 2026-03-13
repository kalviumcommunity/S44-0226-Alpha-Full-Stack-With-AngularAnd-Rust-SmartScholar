import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router, RouterModule } from '@angular/router';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './register.html',
  styleUrls: ['./register.css'],
})
export class Register {
  fullName: string = '';
  email: string = '';
  password: string = '';

  constructor(private router: Router) {}

  register() {
    if (!this.fullName || !this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    console.log('Register:', this.fullName, this.email);

    // later you will call backend API here

    alert('Account created successfully');

    this.router.navigate(['/login']);
  }
}
