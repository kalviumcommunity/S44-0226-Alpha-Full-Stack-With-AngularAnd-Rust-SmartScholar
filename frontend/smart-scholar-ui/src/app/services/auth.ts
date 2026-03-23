import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class Auth {
  private TOKEN_KEY = 'token';

  login(token: string) {
    localStorage.setItem(this.TOKEN_KEY, token);
  }

  logout() {
    localStorage.removeItem(this.TOKEN_KEY);
  }

  getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY);
  }

  isLoggedIn(): boolean {
    return !!this.getToken();
  }

  getRole(): number | null {
    return Number(localStorage.getItem('role'));
    // const token = this.getToken();

    // if (!token) return null;

    // try {
    //   const payload = JSON.parse(atob(token.split('.')[1]));

    //   return payload.role_id;
    // } catch {
    //   return null;
    // }
  }
}
