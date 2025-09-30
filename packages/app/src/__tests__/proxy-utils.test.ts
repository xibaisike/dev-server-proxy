/// <reference types="jest" />
import { applyPathRewrite, getLocationMatch } from '../services/proxy-server';

describe('applyPathRewrite', () => {
  test('rewrites path using regex patterns', () => {
    const rewrite = { '^/api': '' };
    expect(applyPathRewrite('/api/users', rewrite)).toBe('/users');
  });

  test('ignores invalid regex', () => {
    const rewrite: any = { '(invalid': '/ok' };
    expect(applyPathRewrite('/api', rewrite)).toBe('/api');
  });
});

describe('getLocationMatch', () => {
  const locations = [
    { rule: '= /', target: 'A' },
    { rule: '^~ /api', target: 'B' },
    { rule: '~ \\.(css|js)$', target: 'C' },
    { rule: '/', target: 'D' }
  ];

  test('exact match wins', () => {
    const loc = getLocationMatch(locations, '/');
    expect(loc?.target).toBe('A');
  });

  test('prefix match', () => {
    const loc = getLocationMatch(locations, '/api/users');
    expect(loc?.target).toBe('B');
  });

  test('regex match', () => {
    const loc = getLocationMatch(locations, '/style.css');
    expect(loc?.target).toBe('C');
  });

  test('fallback slash', () => {
    const loc = getLocationMatch(locations, '/other/path');
    expect(loc?.target).toBe('D');
  });
});
