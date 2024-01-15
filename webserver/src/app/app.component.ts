import {AfterViewInit, Component, ViewChild, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { Sort, MatSort, MatSortModule } from '@angular/material/sort';
import { BrowserModule, Title } from '@angular/platform-browser';
import { RouterModule, Routes } from '@angular/router';
import countryData from '../assets/parsed_data.json';

interface Countries {
  countries: Country[];
}

interface Country {
  tag: string;
  name: string;
  total_development: number;
  real_development: number;
  gp_score: number;
  powers_earned: number[];
  technology: number[];
  ideas: any[];
  total_ideas: number;
  current_manpower: number;
  max_manpower: number;
  average_monarch: number[];
  income: number;
  number_provinces: number;
  number_buildings: number;
  buildings_value: number;
  buildings_per_province: number;
  innovativeness: number;
  absolutism: number;
  average_development: number;
  average_development_real: number;
  player: any;
  army_professionalism: number;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
  imports: [CommonModule, FormsModule, RouterModule, MatTableModule, MatSortModule, MatPaginatorModule],
  standalone: true,
})

export class AppComponent implements AfterViewInit {
  name = 'Angular';
  displayedColumns: string[] = ['country_name', 'total_dev', 'real_dev', 'gp_score', 'total_mana', 'tech', 'total_ideas', 'curr_manpower', 'max_manpower', 'avg_monarch', 'income', 'provinces', 'num_buildings', 'buildings_value', 'buildings_per_province', 'inno', 'absolutism', 'avg_dev', 'avg_dev_real', 'army_professionalism', 'player'];
  filter = {player: false};
  filteredCountries: Country[] = [];
  countries: Country[] = countryData.countries;
  dataSource = new MatTableDataSource<Country>(countryData.countries)

  @ViewChild(MatSort) sort: MatSort;
  @ViewChild(MatPaginator) paginator: MatPaginator;

  public constructor(private titleService: Title) {
    this.titleService.setTitle("EU4 Stats");
  }

  ngAfterViewInit() {
    this.filteredCountries = this.countries.filter((x: any) => (
      x.player != '' || !this.filter.player));
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
  }

  filterChange() {
    this.filteredCountries = this.countries.filter((x: any) => (
      x.player != null || !this.filter.player));
    this.dataSource = new MatTableDataSource<Country>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }

  sortData(sort: Sort) {
    const data = this.countries.slice();
    if (!sort.active || sort.direction === '') {
      this.countries = data;
      return;
    }

    data.sort((a, b) => {
      const isAsc = sort.direction === 'asc';
      switch (sort.active) {
        case 'country_name':
          return compare(a.name, b.name, isAsc);
        case 'total_dev':
          return compare(a.total_development, b.total_development, isAsc);
        case 'real_dev':
          return compare(a.real_development, b.real_development, isAsc);
        case 'gp_score':
          return compare(a.gp_score, b.gp_score, isAsc);
        case 'total_mana':
          return compareTotal(a.powers_earned, b.powers_earned, isAsc);
        case 'tech':
          return compareTech(a.technology, b.technology, isAsc);
        case 'total_ideas':
          return compare(a.total_ideas, b.total_ideas, isAsc);
        case 'curr_manpower':
          return compare(a.current_manpower, b.current_manpower, isAsc);
        case 'max_manpower':
          return compare(a.max_manpower, b.max_manpower, isAsc);
        case 'avg_monarch':
          return compareTotal(a.average_monarch, b.average_monarch, isAsc);
        case 'income':
          return compare(a.income, b.income, isAsc);
        case 'provinces':
          return compare(a.number_provinces, b.number_provinces, isAsc);
        case 'num_buildings':
          return compare(a.number_buildings, b.number_buildings, isAsc);
        case 'buildings_value':
          return compare(a.buildings_value, b.buildings_value, isAsc);
        case 'buildings_per_province':
          return compare(a.buildings_per_province, b.buildings_per_province, isAsc);
        case 'inno':
          return compare(a.innovativeness, b.innovativeness, isAsc);
        case 'absolutism':
          return compare(a.absolutism, b.absolutism, isAsc);
        case 'avg_dev':
          return compare(a.average_development, b.average_development, isAsc);
        case 'avg_dev_real':
          return compare(a.average_development_real, b.average_development_real, isAsc);
        case 'army_professionalism':
          return compare(a.army_professionalism, b.army_professionalism, isAsc);
        case 'player':
          return compare(a.player, b.player, isAsc);
        default:
          return 0;
      }
    });
    this.filteredCountries = data.filter((x: any) => (
      x.player != null || !this.filter.player));
    this.dataSource = new MatTableDataSource<Country>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }
}

function compare(a: number | string | null, b: number | string | null, isAsc: boolean) {
  if (a === null) {
    return isAsc ? 1 : -1;
  }
  if (b === null) {
    return isAsc ? -1 : 1;
  }
  return (a < b ? -1 : 1) * (isAsc ? 1 : -1);
}

function compareTotal(a: number[], b: number[], isAsc: boolean) {
  const aValue = a[0] + a[1] + a[2];
  const bValue = b[0] + b[1] + b[2];
  return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1); 
}

function compareTech(a: number[], b: number[], isAsc: boolean) {
  const aValue = a[0] + a[1] + a[2];
  const bValue = b[0] + b[1] + b[2];
  if (aValue != bValue) {
    return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (a[0] != b[0]) {
    return (a[0] < b[0] ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (a[1] != b[1]) {
    return (a[1] < b[1] ? -1 : 1) * (isAsc ? 1 : -1);
  }
  return (a[2] < b[2] ? -1 : 1) * (isAsc ? 1 : -1);
}