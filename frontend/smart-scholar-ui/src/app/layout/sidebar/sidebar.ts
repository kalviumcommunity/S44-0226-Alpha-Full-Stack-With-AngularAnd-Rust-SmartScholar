import { Component } from '@angular/core';
import { Auth } from '../../services/auth';

@Component({
  selector: 'app-sidebar',
  imports: [],
  templateUrl: './sidebar.html',
  styleUrl: './sidebar.css',
})
export class Sidebar {
  constructor(public auth: Auth) {}
}
