import {AfterViewInit, Component, ViewChild, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { Sort, MatSort, MatSortModule } from '@angular/material/sort';
import { BrowserModule, Title } from '@angular/platform-browser';
import { RouterModule, Routes } from '@angular/router';
import countryData from '../assets/parsed_data.json';

interface Country {
  tag: String;
  total_dev: Number;
  real_dev: Number;
  gp_score: Number;
  total_mana: String;
  tech: String;
  total_ideas: Number;
  curr_manpower: Number;
  max_manpower: Number;
  avg_monarch: String;
  income: Number;
  provinces: Number;
  num_buildings: Number;
  buildings_value: Number;
  buildings_per_province: Number;
  inno: Number;
  absolutism: Number;
  avg_dev: Number;
  avg_dev_real: Number;
  player: String;
  army_professionalism: Number;
  country_name: String;
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
  countries: Country[] = countryData;
  dataSource = new MatTableDataSource<Country>(countryData)

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
      x.player != '' || !this.filter.player));
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
          return compare(a.country_name, b.country_name, isAsc);
        case 'total_dev':
          return compare(a.total_dev, b.total_dev, isAsc);
        case 'real_dev':
          return compare(a.real_dev, b.real_dev, isAsc);
        case 'gp_score':
          return compare(a.gp_score, b.gp_score, isAsc);
        case 'total_mana':
          return compareWithBreaks(a.total_mana, b.total_mana, isAsc);
        case 'tech':
          return compareTech(a.tech, b.tech, isAsc);
        case 'total_ideas':
          return compare(a.total_ideas, b.total_ideas, isAsc);
        case 'curr_manpower':
          return compare(a.curr_manpower, b.curr_manpower, isAsc);
        case 'max_manpower':
          return compare(a.max_manpower, b.max_manpower, isAsc);
        case 'avg_monarch':
          return compareWithBreaks(a.avg_monarch, b.avg_monarch, isAsc);
        case 'income':
          return compare(a.income, b.income, isAsc);
        case 'provinces':
          return compare(a.provinces, b.provinces, isAsc);
        case 'num_buildings':
          return compare(a.num_buildings, b.num_buildings, isAsc);
        case 'buildings_value':
          return compare(a.buildings_value, b.buildings_value, isAsc);
        case 'buildings_per_province':
          return compare(a.buildings_per_province, b.buildings_per_province, isAsc);
        case 'inno':
          return compare(a.inno, b.inno, isAsc);
        case 'absolutism':
          return compare(a.absolutism, b.absolutism, isAsc);
        case 'avg_dev':
          return compare(a.avg_dev, b.avg_dev, isAsc);
        case 'avg_dev_real':
          return compare(a.avg_dev_real, b.avg_dev_real, isAsc);
        case 'army_professionalism':
          return compare(a.army_professionalism, b.army_professionalism, isAsc);
        case 'player':
          return compare(a.player, b.player, isAsc);
        default:
          return 0;
      }
    });
    this.filteredCountries = data.filter((x: any) => (
      x.player != '' || !this.filter.player));
    this.dataSource = new MatTableDataSource<Country>(this.filteredCountries);
    this.dataSource.paginator = this.paginator;
  }
}

function compare(a: Number | String, b: Number | String, isAsc: boolean) {
  return (a < b ? -1 : 1) * (isAsc ? 1 : -1);
}

function compareWithBreaks(a: String, b: String, isAsc: boolean) {
  const aIndex = a.indexOf('<br>');
  const bIndex = b.indexOf('<br>');
  const aValue = parseFloat(a.substring(0, aIndex));
  const bValue = parseFloat(b.substring(0, bIndex));
  return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1);
}

function compareTech(a: String, b: String, isAsc: boolean) {
  const aTechs = a.split('/');
  const bTechs = b.split('/');
  const aValue = parseInt(aTechs[0]) + parseInt(aTechs[1]) + parseInt(aTechs[2]);
  const bValue = parseInt(bTechs[0]) + parseInt(bTechs[1]) + parseInt(bTechs[2]);
  if (aValue != bValue) {
    return (aValue < bValue ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (aTechs[0] != bTechs[0]) {
    return (parseInt(aTechs[0]) < parseInt(bTechs[0]) ? -1 : 1) * (isAsc ? 1 : -1);
  }
  if (aTechs[1] != bTechs[1]) {
    return (parseInt(aTechs[1]) < parseInt(bTechs[1]) ? -1 : 1) * (isAsc ? 1 : -1);
  }
  return (parseInt(aTechs[2]) < parseInt(bTechs[2]) ? -1 : 1) * (isAsc ? 1 : -1);
}