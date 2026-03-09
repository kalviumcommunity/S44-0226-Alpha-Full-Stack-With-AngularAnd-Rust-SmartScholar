import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './register.html',
  styleUrl: './register.css',
})
export class Register {
  fullName = '';
  email = '';
  password = '';

  register() {
    if (!this.fullName || !this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    console.log('Register:', this.fullName, this.email);
  }
}
