/* tslint:disable */
/* eslint-disable */
/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit the class manually.
 *
 * Jellyfin API
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */


import { SeriesTimerInfoDto } from './series-timer-info-dto';

/**
 * 
 * @export
 * @interface SeriesTimerInfoDtoQueryResult
 */
export interface SeriesTimerInfoDtoQueryResult {
    /**
     * Gets or sets the items.
     * @type {Array<SeriesTimerInfoDto>}
     * @memberof SeriesTimerInfoDtoQueryResult
     */
    'Items'?: Array<SeriesTimerInfoDto> | null;
    /**
     * Gets or sets the total number of records available.
     * @type {number}
     * @memberof SeriesTimerInfoDtoQueryResult
     */
    'TotalRecordCount'?: number;
    /**
     * Gets or sets the index of the first record in Items.
     * @type {number}
     * @memberof SeriesTimerInfoDtoQueryResult
     */
    'StartIndex'?: number;
}

