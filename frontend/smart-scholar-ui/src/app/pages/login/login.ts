import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router, RouterModule } from '@angular/router';
import { Auth } from '../../services/auth';

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

  constructor(
    private authService: Auth,
    private router: Router,
  ) {}

  login() {
    if (!this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    // temporary token until backend login API
    this.authService.login('demo-token');

    this.router.navigate(['/']);
  }
}
