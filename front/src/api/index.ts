import axios from 'axios';

export default {
  async getMessage(): Promise<string> {
    return (await axios.get('/message')).data;
  },
};
