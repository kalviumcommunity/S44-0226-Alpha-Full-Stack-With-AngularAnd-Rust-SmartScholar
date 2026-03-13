import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Router, RouterModule } from '@angular/router';
import { Auth } from '../../services/auth';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule, RouterModule],
  templateUrl: './login.html',
  styleUrls: ['./login.css'],
})
export class Login {
  email: string = '';
  password: string = '';

  constructor(
    private authService: Auth,
    private router: Router,
  ) {}

  login() {
    if (!this.email || !this.password) {
      alert('Please fill all fields');
      return;
    }

    // TEMPORARY until backend login API exists
    const demoToken = 'demo-token';

    this.authService.login(demoToken);

    // redirect to dashboard
    this.router.navigate(['/']);
  }
}
