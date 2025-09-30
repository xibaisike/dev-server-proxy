/// <reference types="jest" />
import { loadConfigFromPath, logEvent } from '../services/proxy-server';
import fs from 'fs';
import path from 'path';

describe('loadConfigFromPath', () => {
  test('loads config module from given path', async () => {
    const tmpPath = path.resolve(__dirname, '../proxy.config.test.njs');
    const content = `export default { port: 12345, locations: [] }`;
    fs.writeFileSync(tmpPath, content);
    const cfg = await loadConfigFromPath(tmpPath);
    expect(cfg.port).toBe(12345);
    fs.unlinkSync(tmpPath);
  });
});

describe('logEvent', () => {
  test('calls fs.appendFile (spy)', () => {
    const appendSpy = jest.spyOn(fs, 'appendFile').mockImplementation((p, d, cb) => cb && cb(null));
    logEvent({ event: 'proxy', reqPath: '/a', proxyPath: 'http://x/a', status: 200 });
    expect(appendSpy).toHaveBeenCalled();
    appendSpy.mockRestore();
  });
});
